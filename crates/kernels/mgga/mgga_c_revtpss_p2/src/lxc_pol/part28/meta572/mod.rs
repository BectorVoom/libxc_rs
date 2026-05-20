//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2034;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta572<F: Float>(t3058: F, t8521: F, t7135: F, t989: F, t25625: F, t7166: F, t11213: F, t1976: F, t11711: F, t25517: F, t11865: F, t25516: F, t11874: F, t27492: F, t11988: F, t7132: F, t3196: F, t7131: F, t11648: F, t7122: F, t25512: F, t3173: F, t11916: F, t25509: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93502, t93509, t93521, t93528, t93541, t93543) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2034::<F>(t3058, t8521, t7135, t989, t25625, t7166, t11213, t1976, t11711, t25517, t11865, t25516);
        let (t93548, t93555, t93561, t93564, t93570, t93573) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2035::<F>(t11874, t27492, t11988, t7132, t3196, t7131, t11648, t7122, t25512, t3173, t11916, t25509);
    (t93502, t93509, t93521, t93528, t93541, t93543, t93548, t93555, t93561, t93564, t93570, t93573)
}
