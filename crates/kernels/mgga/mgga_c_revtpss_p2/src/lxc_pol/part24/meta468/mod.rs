//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1444;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta468<F: Float>(t10722: F, t5993: F, t40593: F, t6037: F, t124: F, t6016: F, t10744: F, t18418: F, t808: F, t10886: F, t18599: F, t1544: F, t1559: F, t40834: F, t854: F, t18608: F, t18352: F, t2710: F, t2713: F, t6030: F, t18419: F, t9775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61677, t61699, t61715, t61797, t61833, t61837) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1444::<F>(t10722, t5993, t40593, t6037, t124, t6016, t10744, t18418, t808, t10886, t18599, t1544, t1559);
        let (t61839, t61877, t61888, t61890, t61892) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1445::<F>(t40834, t61837, t854, t10886, t18608, t808, t18352, t2710, t2713, t10722, t6030, t18419, t9775);
    (t61677, t61699, t61715, t61797, t61833, t61837, t61839, t61877, t61888, t61890, t61892)
}
