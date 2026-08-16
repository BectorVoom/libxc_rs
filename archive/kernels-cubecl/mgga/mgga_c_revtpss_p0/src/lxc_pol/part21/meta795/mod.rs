//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta795 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2875;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2876;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta795<F: Float>(t15397: F, t41583: F, t2874: F, t2918: F, t4632: F, t15534: F, t3022: F, t1100: F, t3329: F, t15537: F, t3007: F, t981: F, t11396: F, t4719: F, t15566: F, t5023: F, t52170: F, t52174: F, t52176: F, t52178: F, t52180: F, t41832: F, t4732: F, t11524: F, t15525: F, t11299: F, t11300: F, t1610: F, t11112: F, t15101: F, t11116: F, t15421: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52182, t52185, t52187, t52188, t52194) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2875::<F>(t15397, t41583, t2874, t2918, t4632, t15534, t3022, t1100, t3329, t15537, t3007, t981);
        let (t52196, t52197) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2876::<F>(t11396, t4719, t15566, t5023, t52170, t52174, t52176, t52178, t52180, t52182, t52185, t52187, t52188, t52194);
        let (t52201, t52204, t52207, t52209, t52211) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2877::<F>(t41832, t4732, t981, t11524, t15525, t11299, t11300, t1610, t11112, t15101, t11116, t15421);
    (t52182, t52185, t52187, t52194, t52196, t52197, t52201, t52204, t52207, t52209, t52211)
}
