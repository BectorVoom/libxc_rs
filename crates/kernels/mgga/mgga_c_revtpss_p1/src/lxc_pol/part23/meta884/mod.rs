//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta884 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2796;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta884<F: Float>(t22307: F, t545: F, t689: F, t869: F, t14239: F, t14242: F, t10023: F, t22314: F, t2470: F, t13790: F, t5658: F, t10022: F, t2782: F, t1882: F, t5710: F, t4086: F, t543: F, t74973: F, t1398: F, t6888: F, t786: F, t4104: F, t23037: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t75174, t75176, t75179, t75190) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2796::<F>(t22307, t545, t689, t869, t14239, t14242, t10023, t22314, t2470, t13790, t5658, t10022, t2782);
        let (t75205, t75215, t75219, t75251, t75252, t75269) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2797::<F>(t1882, t5710, t2782, t4086, t543, t74973, t1398, t6888, t786, t4104, t23037, t10022);
    (t75174, t75176, t75179, t75190, t75205, t75215, t75219, t75251, t75252, t75269)
}
