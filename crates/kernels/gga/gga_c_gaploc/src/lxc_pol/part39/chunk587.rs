//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 587/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk587<F: Float>(t1967: F, t9889: F, t7810: F, t3209: F, t5241: F, t590: F, t1890: F, t3234: F, t3243: F, t1628: F, t3318: F, t1966: F, t1991: F, t1998: F, t5640: F, t6148: F, t813: F, t833: F, t9870: F, t9873: F, t9876: F, t9880: F, t9883: F, t9886: F) -> (F, F, F, F) {
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    let t9892 = F::cast_from(0.38342925953920749676e0_f64) * t9891;
    let t9893 = t5241 * t3209;
    let t9894 = t9893 * t590;
    let t9897 = t1890 * t3234;
    let t9898 = t9897 * t590;
    let t9901 = t3243 * t590;
    let t9904 = t1628 * t3318;
    let t9907 = -F::cast_from(0.23005755572352449806e1_f64) * t1998 * t9870 - F::cast_from(0.7988109573733489516e-2_f64) * t9873 + F::cast_from(0.69017266717057349418e1_f64) * t6148 * t9876 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t9880 - F::cast_from(0.11502877786176224903e2_f64) * t1998 * t9883 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t9886 - t9892 + F::cast_from(0.15337170381568299871e1_f64) * t5640 * t9894 - F::cast_from(0.51123901271894332902e0_f64) * t1966 * t9898 + F::cast_from(0.1022478025437886658e1_f64) * t1991 * t9901 + F::cast_from(0.30674340763136599741e1_f64) * t833 * t9904;
    (t9891, t9892, t9893, t9907)
}
