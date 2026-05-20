//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk665;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk666;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta125<F: Float>(t1469: F, t2852: F, t2857: F, t1596: F, t914: F, t1600: F, t2880: F, t2897: F, t1606: F, t698: F, t1614: F, t945: F, t1626: F, t964: F, t1633: F, t3014: F, t300: F, t2986: F, t1646: F, t993: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4573, t4578, t4590, t4598, t4614, t4620, t4647) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk665::<F>(t1469, t2852, t2857, t1596, t914, t1600, t2880, t2897, t1606, t698, t1614, t945);
        let (t4685, t4711, t4719) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk666::<F>(t1626, t964, t1633, t3014, t300);
        let (t4724, t4746) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk667::<F>(t1633, t2986, t1646, t993);
    (t4573, t4578, t4590, t4598, t4614, t4620, t4647, t4685, t4711, t4719, t4724, t4746)
}
