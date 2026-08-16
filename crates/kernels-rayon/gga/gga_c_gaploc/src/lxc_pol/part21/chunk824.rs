//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 824/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk824(t4323: f64, t7906: f64, t105: f64, t1079: f64, t1083: f64, t1212: f64, t1349: f64, t1358: f64, t2757: f64, t2793: f64, t380: f64, t419: f64, t7864: f64, t7880: f64, t7889: f64, t7894: f64, t7898: f64, t7902: f64, t989: f64, t994: f64) -> f64 {
    let t7907 = t4323 * t7906;
    let t7910 = 0.12646669615856066488e-1_f64 * t1079 * t989 - 0.28455006635676149599e-1_f64 * t1212 * t994 + 0.28455006635676149599e-1_f64 * t105 * t7864 + 0.7588001769513639893e-1_f64 * t1083 * t989 + 0.7588001769513639893e-1_f64 * t380 * t2757 + 0.28455006635676149599e-1_f64 * t1212 * t989 + 0.56910013271352299198e-1_f64 * t419 * t2757 - 0.56910013271352299198e-1_f64 * t419 * t2793 - 0.12646669615856066488e-1_f64 * t1079 * t994 - 0.28455006635676149599e-1_f64 * t105 * t7880 - 0.7588001769513639893e-1_f64 * t1083 * t994 - 0.7588001769513639893e-1_f64 * t380 * t2793 + 0.18970004423784099732e-1_f64 * t1358 * t7889 - 0.12646669615856066488e-1_f64 * t1358 * t7894 + 0.63233348079280332442e-2_f64 * t1349 * t7898 - 0.63233348079280332442e-2_f64 * t1358 * t7902 - 0.63233348079280332442e-2_f64 * t1349 * t7907;
    t7910
}
