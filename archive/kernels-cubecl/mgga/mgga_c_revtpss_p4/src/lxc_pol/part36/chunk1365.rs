//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1365/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1365<F: Float>(t105579: F, t112774: F, t112843: F, t116500: F, t1769: F, t1828: F, t24509: F, t24900: F, t25022: F, t26949: F, t26969: F, t29136: F, t29207: F, t29227: F, t29275: F, t30736: F, t30739: F, t30747: F, t30752: F, t30758: F, t30767: F, t30840: F, t3783: F, t6574: F, t6703: F, t6745: F, t7602: F, t7632: F, t7636: F, t7643: F, t7651: F, t7652: F, t8198: F, t8213: F, t97397: F) -> F {
    let t116649 = -F::cast_from(0.26020884564615598386e1_f64) * t97397 * t116500 * t3783 - F::cast_from(0.15612530738769359031e2_f64) * t7636 * t26969 * t30767 * t1769 + F::cast_from(0.39512695097613069591e1_f64) * t7632 * t24509 + F::cast_from(0.26020884564615598386e1_f64) * t7651 * t7652 * t30840 * t1828 + F::cast_from(0.19756347548806534796e1_f64) * t7602 * t24900 + F::cast_from(0.39512695097613069591e1_f64) * t29227 * t6703 - F::cast_from(0.65854491829355115987e0_f64) * t7602 * t25022 - F::cast_from(0.10408353825846239354e2_f64) * t29136 * t30758 + F::cast_from(0.26020884564615598386e1_f64) * t29136 * t30736 + F::cast_from(0.39512695097613069591e1_f64) * t105579 * t6574 - F::cast_from(0.26020884564615598386e1_f64) * t112774 * t8198 - F::cast_from(0.10408353825846239354e2_f64) * t7643 * t7652 * t30747 * t1828 - F::cast_from(0.19756347548806534796e1_f64) * t29207 * t6745 - F::cast_from(0.26020884564615598386e1_f64) * t29275 * t30752 + F::cast_from(0.15612530738769359031e2_f64) * t26949 * t7652 * t30739 * t1828 - F::cast_from(0.26020884564615598386e1_f64) * t112843 * t8213;
    t116649
}
