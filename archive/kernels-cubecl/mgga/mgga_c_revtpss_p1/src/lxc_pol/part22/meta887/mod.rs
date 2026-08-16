//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta887 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3074;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta887<F: Float>(t1633: F, t3012: F, t2986: F, t4682: F, t11465: F, t1626: F, t15234: F, t3014: F, t11509: F, t4707: F, t11385: F, t1609: F, t2873: F, t4587: F, t11298: F, t1596: F, t11466: F, t11299: F, t15494: F, t964: F, t3011: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52430, t52440, t52443, t52452, t52459, t52482) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3074::<F>(t1633, t3012, t2986, t4682, t11465, t1626, t15234, t3014, t11509, t4707, t11385, t1609);
        let (t52505, t52508, t52511, t52514, t52522, t52637) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3075::<F>(t2873, t4587, t11298, t1596, t11466, t1633, t11299, t1609, t15494, t964, t3011, t4682);
    (t52430, t52440, t52443, t52452, t52459, t52482, t52505, t52508, t52511, t52514, t52522, t52637)
}
