//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1945;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta549<F: Float>(t1984: F, t29807: F, t359: F, t1646: F, t993: F, t378: F, t1652: F, t1696: F, t1983: F, t1986: F, t25605: F, t25611: F, t25629: F, t27550: F, t27568: F, t27621: F, t27699: F, t29728: F, t29732: F, t29740: F, t29744: F, t29748: F, t29752: F, t29760: F, t29809: F, t29812: F, t29818: F, t29822: F, t29826: F, t342: F, t6259: F, t7102: F, t7144: F, t7151: F, t7159: F, t7167: F, t7833: F, t1651: F, t7817: F, t7145: F, t25672: F, t3304: F, t6305: F, t3318: F, t7168: F, t1695: F, t7160: F, t1976: F, t6244: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29830, t29833, t29834, t29835, t29838) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1945::<F>(t1984, t29807, t359, t1646, t993, t378, t1652, t1696, t1983, t1986, t25605, t25611, t25629, t27550, t27568, t27621, t27699, t29728, t29732, t29740, t29744, t29748, t29752, t29760, t29809, t29812, t29818, t29822, t29826, t342, t6259, t7102, t7144, t7151, t7159, t7167, t7833);
        let (t29843, t29844, t29848, t29852, t29865, t29866, t29871) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1946::<F>(t1651, t7817, t7145, t25672, t3304, t6305, t3318, t7168, t1695, t7160, t1976, t6244);
    (t29830, t29833, t29834, t29835, t29838, t29843, t29844, t29848, t29852, t29865, t29866, t29871)
}
