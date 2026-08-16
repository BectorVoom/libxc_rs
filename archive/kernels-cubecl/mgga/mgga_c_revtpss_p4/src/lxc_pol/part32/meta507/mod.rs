//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1796;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta507<F: Float>(t5920: F, t93: F, t1843: F, t7983: F, t5542: F, t8108: F, t2097: F, t6861: F, t4003: F, t26079: F, t26321: F, t26324: F, t26325: F, t26328: F, t27921: F, t27926: F, t27929: F, t27953: F, t27955: F, t30048: F, t30050: F, t26310: F, t26312: F, t27924: F, t27937: F, t30035: F, t30037: F, t30039: F, t30041: F, t30043: F, t30045: F) -> (F, F, F, F, F, F, F) {
        let (t30143, t30209, t30218, t30225, t30226, t30227, t30246) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1796::<F>(t5920, t93, t1843, t7983, t5542, t8108, t2097, t6861, t4003, t26079, t26321, t26324, t26325, t26328, t27921, t27926, t27929, t27953, t27955, t30048, t30050);
        let t30247 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1797::<F>(t26310, t26312, t27924, t27937, t30035, t30037, t30039, t30041, t30043, t30045, t30246);
    (t30143, t30209, t30218, t30225, t30226, t30227, t30247)
}
