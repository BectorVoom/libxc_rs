//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta868 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2764;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta868<F: Float>(t22025: F, t2661: F, t5675: F, t9934: F, t6836: F, t9940: F, t1353: F, t13767: F, t13768: F, t5591: F, t21969: F, t221: F, t3978: F, t3979: F, t4010: F, t6816: F, t22027: F, t9775: F, t22252: F, t3992: F, t543: F, t550: F, t22263: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t73985, t73994, t73998, t74010) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2764::<F>(t22025, t2661, t5675, t9934, t6836, t9940, t1353, t13767, t13768, t5591, t21969, t221, t3978, t3979);
        let (t74012, t74015, t74017, t74022, t74024) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2765::<F>(t4010, t6816, t1353, t13767, t2661, t22027, t9775, t22252, t3992, t543, t550, t22263);
    (t73985, t73994, t73998, t74010, t74012, t74015, t74017, t74022, t74024)
}
