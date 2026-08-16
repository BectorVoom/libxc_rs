//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2768;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta777(t50880: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t50857: f64, t50861: f64, t50864: f64, t50866: f64, t50869: f64, t50871: f64, t50872: f64, t50874: f64, t50875: f64, t50876: f64, t50879: f64, t14322: f64, t2626: f64, t10326: f64, t4401: f64, t4402: f64, t4398: f64, t9425: f64, t10555: f64, t14613: f64, t10565: f64, t1532: f64, t9419: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50881, t50882) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2768(t50880, t40067, t40072, t40167, t40171, t40184, t50857, t50861, t50864, t50866, t50869, t50871, t50872, t50874, t50875, t50876, t50879);
        let (t50884, t50887, t50889, t50891, t50892, t50893) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2769(t14322, t2626, t10326, t4401, t4402, t4398, t9425, t10555, t14613, t10565, t1532, t9419);
    (t50881, t50882, t50884, t50887, t50889, t50891, t50892, t50893)
}
