//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1221/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1221<F: Float>(t36830: F, t36842: F, t36860: F, t36872: F, t36887: F, t36906: F, t36926: F, t36945: F, t36966: F, t36985: F, t37005: F, t37024: F, t37043: F, t37058: F, t37078: F, t37095: F, t37116: F, t37134: F, t37154: F, t37172: F, t37192: F, t37208: F, t37227: F, t37245: F, t37266: F, t37285: F, t37305: F, t37324: F, t37344: F, t37358: F, t37377: F, t37396: F, t37419: F, t37433: F, t37453: F, t37471: F, t37491: F, t37510: F, t37530: F, t37547: F, t37568: F, t37587: F, t37603: F, t37617: F, t37637: F, t37654: F, t37674: F, t37688: F, t37710: F, t37727: F, t37747: F, t37766: F, t37785: F, t37804: F, t37824: F, t37843: F, t37863: F, t37881: F, t37901: F, t37919: F, t37939: F, t37958: F, t37977: F, t37995: F) -> F {
    let t38001 = t37939 + t37637 + t36842 + t37804 + t37024 + t37766 + t37134 + t37491 + t37901 + t37043 + t37227 + t37419 + t36872 + t37919 + t36906 + t37324 + t37603 + t37058 + t37305 + t37510 + t37245 + t37688 + t37078 + t37674 + t37863 + t37266 + t37617 + t37977 + t37285 + t37116 + t37208 + t37344 + t36860 + t37377 + t37881 + t37995 + t36887 + t37727 + t37396 + t37587 + t37843 + t37358 + t37747 + t37453 + t37530 + t36926 + t36985 + t37433 + t37192 + t37172 + t37824 + t37095 + t37471 + t36945 + t37568 + t37154 + t36830 + t37654 + t37005 + t36966 + t37710 + t37785 + t37547 + t37958;
    t38001
}
