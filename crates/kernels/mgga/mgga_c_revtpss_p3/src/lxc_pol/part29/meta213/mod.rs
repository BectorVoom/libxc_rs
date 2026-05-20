//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk949;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk950;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta213<F: Float>(t4669: F, t954: F, t1621: F, t2970: F, t953: F, t2848: F, t2974: F, t4571: F, t4576: F, t4581: F, t4585: F, t324: F, t1626: F, t964: F, t1634: F, t972: F, t2906: F, t2994: F, t3001: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t4623: F, t4626: F, t4629: F, t973: F, t1633: F, t3014: F, t1622: F, t2938: F, t2943: F, t2968: F, t2982: F, t2987: F, t3012: F, t311: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4644: F, t4647: F, t4652: F, t946: F, t955: F, t965: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4670, t4673, t4674, t4682, t4683) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk949::<F>(t4669, t954, t1621, t2970, t953, t2848, t2974, t4571, t4576, t4581, t4585, t324);
        let (t4685, t4690, t4707) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk950::<F>(t1626, t964, t1634, t972, t2848, t2906, t2994, t3001, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620, t4623, t4626, t4629);
        let (t4708, t4711, t4712, t4715) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk951::<F>(t4707, t973, t1633, t3014, t972, t1622, t1634, t2938, t2943, t2968, t2982, t2987, t3012, t311, t4589, t4592, t4594, t4597, t4634, t4638, t4644, t4647, t4652, t4670, t4674, t4683, t4685, t4690, t946, t955, t965, t974);
    (t4670, t4673, t4674, t4682, t4683, t4685, t4690, t4707, t4708, t4711, t4712, t4715)
}
