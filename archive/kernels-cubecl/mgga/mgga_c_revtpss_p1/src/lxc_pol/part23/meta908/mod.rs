//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta908 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2916;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2917;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta908<F: Float>(t23535: F, t2880: F, t918: F, t51914: F, t51915: F, t63240: F, t63242: F, t77663: F, t77667: F, t77670: F, t77672: F, t77674: F, t77676: F, t2897: F, t23540: F, t41401: F, t18979: F, t4606: F, t15113: F, t6120: F, t18950: F, t4598: F, t41382: F, t15107: F, t15110: F, t41246: F, t77499: F, t77503: F, t77505: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t77679, t77681) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2916::<F>(t23535, t2880, t918, t51914, t51915, t63240, t63242, t77663, t77667, t77670, t77672, t77674, t77676);
        let (t77683, t77686, t77688, t77690, t77692, t77695) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2917::<F>(t23535, t2897, t918, t23540, t41401, t18979, t4606, t15113, t6120, t18950, t4598, t41382);
        let (t77698, t77700, t77705) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2918::<F>(t6120, t918, t15107, t15110, t41246, t77499, t77503, t77505, t77683, t77686, t77688, t77690, t77692, t77695);
    (t77679, t77681, t77683, t77686, t77688, t77690, t77692, t77695, t77698, t77700, t77705)
}
