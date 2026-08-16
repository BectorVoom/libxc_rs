//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1820/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1820<F: Float>(t6041: F, t822: F, t6022: F, t72: F, t686: F, t10530: F, t10645: F, t10647: F, t10651: F, t14558: F, t14564: F, t14570: F, t18616: F, t18632: F, t18657: F, t18690: F, t18699: F, t213: F, t234: F, t2815: F, t4424: F, t4494: F, t4504: F, t4514: F, t4526: F, t6017: F, t820: F, t837: F, t879: F) -> (F, F, F, F, F) {
    let t18714 = t822 * t6041;
    let t18718 = t6022 * t72;
    let t18719 = t18718 * t686;
    let t18720 = t10530 * t18719;
    let t18722 = -F::cast_from(0.54878743191129263322e-2_f64) * t18690 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t234 * t18657 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t2815 * t6017 - F::cast_from(0.13009920719177044025e-2_f64) * t14558 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t18699 * t837 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t4526 * t4424 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t4494 * t18632 + F::cast_from(0.26019841438354088051e-1_f64) * t14564 - t10645 - F::cast_from(0.13009920719177044025e-1_f64) * t10647 + t10651 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t18616 - t14570 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t18714 * t837 + F::cast_from(0.19514881078765566037e-1_f64) * t18720;
    (t18714, t18718, t18719, t18720, t18722)
}
