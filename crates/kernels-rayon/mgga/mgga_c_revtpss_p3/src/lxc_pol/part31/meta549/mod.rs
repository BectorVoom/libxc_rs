//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1945;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta549(t1984: f64, t29807: f64, t359: f64, t1646: f64, t993: f64, t378: f64, t1652: f64, t1696: f64, t1983: f64, t1986: f64, t25605: f64, t25611: f64, t25629: f64, t27550: f64, t27568: f64, t27621: f64, t27699: f64, t29728: f64, t29732: f64, t29740: f64, t29744: f64, t29748: f64, t29752: f64, t29760: f64, t29809: f64, t29812: f64, t29818: f64, t29822: f64, t29826: f64, t342: f64, t6259: f64, t7102: f64, t7144: f64, t7151: f64, t7159: f64, t7167: f64, t7833: f64, t1651: f64, t7817: f64, t7145: f64, t25672: f64, t3304: f64, t6305: f64, t3318: f64, t7168: f64, t1695: f64, t7160: f64, t1976: f64, t6244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29830, t29833, t29834, t29835, t29838) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1945(t1984, t29807, t359, t1646, t993, t378, t1652, t1696, t1983, t1986, t25605, t25611, t25629, t27550, t27568, t27621, t27699, t29728, t29732, t29740, t29744, t29748, t29752, t29760, t29809, t29812, t29818, t29822, t29826, t342, t6259, t7102, t7144, t7151, t7159, t7167, t7833);
        let (t29843, t29844, t29848, t29852, t29865, t29866, t29871) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1946(t1651, t7817, t7145, t25672, t3304, t6305, t3318, t7168, t1695, t7160, t1976, t6244);
    (t29830, t29833, t29834, t29835, t29838, t29843, t29844, t29848, t29852, t29865, t29866, t29871)
}
