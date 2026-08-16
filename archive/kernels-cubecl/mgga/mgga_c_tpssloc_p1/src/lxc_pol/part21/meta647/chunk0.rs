//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2441/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2441<F: Float>(t340: F, t625: F, t221: F, t339: F, t344: F, t1887: F, t2262: F, t337: F, t13783: F, t984: F, t10277: F, t343: F) -> (F, F, F, F, F) {
    let t42813 = t625 * t340;
    let t42817 = F::cast_from(0.82304526748971193413e-3_f64) * t339 * t221 * t42813 * t344;
    let t42830 = t2262 * t337 * t1887;
    let t42837 = t13783 * t984;
    let t42841 = t343 * t10277;
    (t42813, t42817, t42830, t42837, t42841)
}
