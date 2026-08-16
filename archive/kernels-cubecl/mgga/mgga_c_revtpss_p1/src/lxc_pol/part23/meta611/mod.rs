//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2275;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2276;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta611<F: Float>(t24285: F, t24322: F, t1150: F, t1131: F, t12230: F, t24220: F, t12227: F, t1744: F, t6486: F, t3479: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24265: F, t24267: F, t24272: F, t24275: F, t12459: F, t12460: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F, t24313: F, t24315: F, t24318: F, t24320: F, t1169: F, t12472: F, t1756: F, t6518: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24323, t24324, t24326, t24327, t24329, t24330, t24331, t24348) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2275::<F>(t24285, t24322, t1150, t1131, t12230, t24220, t12227, t1744, t6486, t3479, t16706, t16876, t20276, t20278, t20280, t20283, t20285, t20287, t24230, t24234, t24265, t24267, t24272, t24275);
        let t24361 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2276::<F>(t12459, t12460, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298, t24313, t24315, t24318, t24320);
        let (t24362, t24363, t24366, t24375) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2277::<F>(t24348, t24361, t1169, t12472, t24330, t1756, t6518);
    (t24323, t24324, t24326, t24327, t24329, t24330, t24331, t24362, t24363, t24366, t24375)
}
