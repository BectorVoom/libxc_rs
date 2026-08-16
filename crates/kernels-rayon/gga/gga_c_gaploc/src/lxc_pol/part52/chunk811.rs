//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 811/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk811(t44294: f64, t6508: f64, t1358: f64, t6507: f64, t2339: f64, t35918: f64, t42581: f64, t42587: f64, t42590: f64, t11182: f64, t2317: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44295 = t6508 * t44294;
    let t44298 = 0.63233348079280332442e-2_f64 * t1358 * t6507 * t44295;
    let t44301 = 0.22131671827748116354e-1_f64 * t1358 * t35918 * t2339;
    let t44302 = 0.18970004423784099733e-1_f64 * t42581;
    let t44305 = 0.142275033178380748e-1_f64 * t42587;
    let t44306 = 0.142275033178380748e-1_f64 * t42590;
    let t44308 = t6525 * t11182 * t2317;
    (t44295, t44298, t44301, t44302, t44305, t44306, t44308)
}
