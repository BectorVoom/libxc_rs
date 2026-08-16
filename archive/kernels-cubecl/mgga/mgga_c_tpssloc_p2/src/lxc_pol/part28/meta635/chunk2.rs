//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2013/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2013<F: Float>(t90805: F, t2085: F, t5286: F, t1824: F, t7191: F, t90837: F, t1352: F, t16123: F, t2089: F, t27074: F, t3851: F, t5250: F, t5334: F, t5344: F, t90801: F, t90807: F, t90812: F, t90816: F, t90821: F, t90825: F, t90829: F, t90832: F, t90835: F, t90840: F) -> (F, F) {
    let t93494 = F::cast_from(0.3289868133696452873e-1_f64) * t90805;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t93517 = F::cast_from(0.10417915756705434098e0_f64) * t90837;
    let t93519 = -F::cast_from(0.3289868133696452873e-1_f64) * t90801 + t93494 - F::cast_from(0.25587863262083522346e0_f64) * t90807 - F::cast_from(0.6579736267392905746e-1_f64) * t90812 + F::cast_from(0.6579736267392905746e-1_f64) * t90816 + F::cast_from(0.6579736267392905746e-1_f64) * t90821 - F::cast_from(0.3289868133696452873e-1_f64) * t90825 - F::cast_from(0.6579736267392905746e-1_f64) * t90829 - F::cast_from(2.0_f64) * t5344 * t93501 * t1352 + F::cast_from(4.0_f64) * t5334 * t93505 * t5250 + F::cast_from(4.0_f64) * t5334 * t93501 * t5250 - t5344 * t27074 * t3851 - F::cast_from(0.9869604401089358619e-1_f64) * t90832 + F::cast_from(0.9869604401089358619e-1_f64) * t90835 + t16123 * t2089 - t93517 - F::cast_from(0.3289868133696452873e-1_f64) * t90840;
    (t93505, t93519)
}
