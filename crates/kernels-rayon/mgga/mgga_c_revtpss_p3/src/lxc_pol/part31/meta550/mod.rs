//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1947;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1948;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1949;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta550(t29871: f64, t7145: f64, t1976: f64, t6234: f64, t6258: f64, t1695: f64, t7810: f64, t7160: f64, t1647: f64, t1696: f64, t1978: f64, t25591: f64, t25651: f64, t25671: f64, t25699: f64, t27419: f64, t27609: f64, t27616: f64, t27661: f64, t29844: f64, t29848: f64, t29852: f64, t29866: f64, t6235: f64, t6245: f64, t6251: f64, t6351: f64, t6393: f64, t7102: f64, t7140: f64, t7144: f64, t7151: f64, t7159: f64, t7812: f64, t7818: f64, t7822: f64, t7825: f64, t7829: f64, t7837: f64, t29838: f64, t1963: f64, t5966: f64, t1544: f64, t1583: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t25445: f64, t27368: f64, t29598: f64, t29704: f64, t4541: f64, t5962: f64, t6075: f64, t6079: f64, t7091: f64, t7783: f64, t892: f64, t265: f64, t393: f64, t1102: f64, t1699: f64, t25713: f64, t27712: f64, t336: f64, t5023: f64, t6396: f64, t6400: f64, t7181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29872, t29875, t29876, t29883, t29884, t29887) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1947(t29871, t7145, t1976, t6234, t6258, t1695, t7810);
        let (t29888, t29893) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1948(t29887, t7160, t1647, t1696, t1978, t25591, t25651, t25671, t25699, t27419, t27609, t27616, t27661, t29844, t29848, t29852, t29866, t29872, t29876, t29884, t6235, t6245, t6251, t6351, t6393, t7102, t7140, t7144, t7151, t7159, t7812, t7818, t7822, t7825, t7829, t7837);
        let (t29894, t29907, t29930) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1949(t29838, t29893, t1963, t5966, t1544, t1583, t1940, t198, t207, t2403, t25445, t27368, t29598, t29704, t4541, t5962, t6075, t6079, t7091, t7783, t892);
        let t29931 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1950(t265, t393, t1102, t1699, t198, t25713, t27712, t29894, t29930, t336, t5023, t6396, t6400, t7181);
    (t29872, t29875, t29876, t29883, t29884, t29887, t29888, t29894, t29907, t29930, t29931)
}
