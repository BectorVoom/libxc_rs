//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1947;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1948;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1949;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta550<F: Float>(t29871: F, t7145: F, t1976: F, t6234: F, t6258: F, t1695: F, t7810: F, t7160: F, t1647: F, t1696: F, t1978: F, t25591: F, t25651: F, t25671: F, t25699: F, t27419: F, t27609: F, t27616: F, t27661: F, t29844: F, t29848: F, t29852: F, t29866: F, t6235: F, t6245: F, t6251: F, t6351: F, t6393: F, t7102: F, t7140: F, t7144: F, t7151: F, t7159: F, t7812: F, t7818: F, t7822: F, t7825: F, t7829: F, t7837: F, t29838: F, t1963: F, t5966: F, t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2403: F, t25445: F, t27368: F, t29598: F, t29704: F, t4541: F, t5962: F, t6075: F, t6079: F, t7091: F, t7783: F, t892: F, t265: F, t393: F, t1102: F, t1699: F, t25713: F, t27712: F, t336: F, t5023: F, t6396: F, t6400: F, t7181: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t29872, t29875, t29876, t29883, t29884, t29887) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1947::<F>(t29871, t7145, t1976, t6234, t6258, t1695, t7810);
        let (t29888, t29893) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1948::<F>(t29887, t7160, t1647, t1696, t1978, t25591, t25651, t25671, t25699, t27419, t27609, t27616, t27661, t29844, t29848, t29852, t29866, t29872, t29876, t29884, t6235, t6245, t6251, t6351, t6393, t7102, t7140, t7144, t7151, t7159, t7812, t7818, t7822, t7825, t7829, t7837);
        let (t29894, t29907, t29930) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1949::<F>(t29838, t29893, t1963, t5966, t1544, t1583, t1940, t198, t207, t2403, t25445, t27368, t29598, t29704, t4541, t5962, t6075, t6079, t7091, t7783, t892);
        let t29931 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1950::<F>(t265, t393, t1102, t1699, t198, t25713, t27712, t29894, t29930, t336, t5023, t6396, t6400, t7181);
    (t29872, t29875, t29876, t29883, t29884, t29887, t29888, t29894, t29907, t29930, t29931)
}
