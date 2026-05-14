//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1182/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1182<F: Float>(t34930: F, t10546: F, t1415: F, t1645: F, t4807: F, t10399: F, t1436: F, t18482: F, t31540: F, t10470: F, t4849: F, t10430: F, t587: F, t589: F, t10438: F, t1391: F) -> (F, F, F, F, F, F, F) {
    let t34931 = 0.59644551483876721719e0 * t34930;
    let t34935 = 0.50050685932590597338e1 * t1415 * t10546 * t1645 * t4807;
    let t34936 = t1436 * t10399;
    let t34937 = 0.51123901271894332902e0 * t34936;
    let t34939 = 0.15889106645266856297e0 * t18482 * t31540;
    let t34941 = 0.51123901271894332902e1 * t4849 * t10470;
    let t34943 = t587 * t589 * t10430;
    let t34944 = 0.51123901271894332902e0 * t34943;
    let t34946 = t587 * t1391 * t10438;
    (t34931, t34935, t34937, t34939, t34941, t34944, t34946)
}
