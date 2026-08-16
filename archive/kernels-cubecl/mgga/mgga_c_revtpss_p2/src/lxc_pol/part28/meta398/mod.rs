//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1504;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1505;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1506;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta398<F: Float>(t14622: F, t4401: F, t2414: F, t4311: F, t10428: F, t1522: F, t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14442: F, t14443: F, t14444: F, t14615: F, t14618: F, t14620: F, t14621: F, t9542: F, t14609: F, t14610: F, t14612: F, t225: F, t73: F, t830: F, t1544: F, t2475: F, t2394: F, t4343: F, t853: F, t775: F, t2430: F, t4416: F, t14468: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t2634: F, t2639: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t833: F, t231: F, t10943: F, t4364: F, t4365: F, t124: F, t1558: F, t10779: F, t2749: F, t10777: F, t125: F, t4423: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14624, t14626, t14628, t14629, t14630) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1504::<F>(t14622, t4401, t2414, t4311, t10428, t1522, t10613, t10592, t10596, t10604, t10611, t14442, t14443, t14444, t14615, t14618, t14620, t14621, t9542);
        let (t14633, t14643, t14648, t14649, t14652) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1505::<F>(t14609, t14610, t14612, t14630, t225, t73, t830, t1544, t2475, t2394, t4343, t853);
        let t14662 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1506::<F>(t14652, t775, t2430, t4416, t14468, t832, t14633, t14643, t14649, t1553, t1555, t227, t229, t2634, t2639, t2642, t4409, t4415, t4417, t4420, t830, t833);
        let (t14663, t14668, t14671, t14673, t14675, t14676) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1507::<F>(t14662, t231, t10943, t4364, t4365, t124, t1558, t10779, t2749, t10777, t125, t4423);
    (t14624, t14626, t14628, t14629, t14648, t14662, t14663, t14668, t14671, t14673, t14675, t14676)
}
