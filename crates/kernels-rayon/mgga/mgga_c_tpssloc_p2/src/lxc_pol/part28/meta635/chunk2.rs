//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2013/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2013(t90805: f64, t2085: f64, t5286: f64, t1824: f64, t7191: f64, t90837: f64, t1352: f64, t16123: f64, t2089: f64, t27074: f64, t3851: f64, t5250: f64, t5334: f64, t5344: f64, t90801: f64, t90807: f64, t90812: f64, t90816: f64, t90821: f64, t90825: f64, t90829: f64, t90832: f64, t90835: f64, t90840: f64) -> (f64, f64) {
    let t93494 = 0.3289868133696452873e-1_f64 * t90805;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t93517 = 0.10417915756705434098e0_f64 * t90837;
    let t93519 = -0.3289868133696452873e-1_f64 * t90801 + t93494 - 0.25587863262083522346e0_f64 * t90807 - 0.6579736267392905746e-1_f64 * t90812 + 0.6579736267392905746e-1_f64 * t90816 + 0.6579736267392905746e-1_f64 * t90821 - 0.3289868133696452873e-1_f64 * t90825 - 0.6579736267392905746e-1_f64 * t90829 - 2.0_f64 * t5344 * t93501 * t1352 + 4.0_f64 * t5334 * t93505 * t5250 + 4.0_f64 * t5334 * t93501 * t5250 - t5344 * t27074 * t3851 - 0.9869604401089358619e-1_f64 * t90832 + 0.9869604401089358619e-1_f64 * t90835 + t16123 * t2089 - t93517 - 0.3289868133696452873e-1_f64 * t90840;
    (t93505, t93519)
}
