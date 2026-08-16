//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1451;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta259<F: Float>(t3829: F, t4011: F, t547: F, t807: F, t2237: F, t240: F, t550: F, t816: F, t1379: F, t2689: F, t3952: F, t1413: F, t3889: F, t9646: F, t2236: F, t66: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9703, t9704, t9705, t9707, t9711, t9712, t9714) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1451::<F>(t3829, t4011, t547, t807, t2237, t240, t550, t816, t1379, t2689, t3952, t1413, t3889);
        let (t9715, t9716, t9718, t9720) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1452::<F>(t547, t9714, t807, t9646, t2236, t66);
    (t9703, t9704, t9705, t9707, t9711, t9712, t9714, t9715, t9716, t9718, t9720)
}
