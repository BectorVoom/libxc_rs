//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 825/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk825<F: Float>(t4323: F, t7906: F, t105: F, t1079: F, t1083: F, t1212: F, t1349: F, t1358: F, t2757: F, t2793: F, t380: F, t419: F, t7864: F, t7880: F, t7889: F, t7894: F, t7898: F, t7902: F, t989: F, t994: F) -> F {
    let t7907 = t4323 * t7906;
    let t7910 = F::new(0.12646669615856066488e-1) * t1079 * t989 - F::new(0.28455006635676149599e-1) * t1212 * t994 + F::new(0.28455006635676149599e-1) * t105 * t7864 + F::new(0.7588001769513639893e-1) * t1083 * t989 + F::new(0.7588001769513639893e-1) * t380 * t2757 + F::new(0.28455006635676149599e-1) * t1212 * t989 + F::new(0.56910013271352299198e-1) * t419 * t2757 - F::new(0.56910013271352299198e-1) * t419 * t2793 - F::new(0.12646669615856066488e-1) * t1079 * t994 - F::new(0.28455006635676149599e-1) * t105 * t7880 - F::new(0.7588001769513639893e-1) * t1083 * t994 - F::new(0.7588001769513639893e-1) * t380 * t2793 + F::new(0.18970004423784099732e-1) * t1358 * t7889 - F::new(0.12646669615856066488e-1) * t1358 * t7894 + F::new(0.63233348079280332442e-2) * t1349 * t7898 - F::new(0.63233348079280332442e-2) * t1358 * t7902 - F::new(0.63233348079280332442e-2) * t1349 * t7907;
    t7910
}
