//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1081/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1081<F: Float>(t3351: F, t6338: F, t10160: F, t23927: F, t2317: F, t6525: F, t8026: F, t1365: F, t23983: F, t25575: F, t4382: F, t986: F, t6470: F, t9074: F, t1063: F, t30189: F, t30199: F, t30207: F, t32053: F, t32055: F, t32057: F, t32059: F, t32062: F, t32066: F, t32067: F, t535: F) -> (F, F) {
    let t32071 = t6338 * t3351;
    let t32072 = 0.11856252764865062333e-2 * t32071;
    let t32073 = t23927 * t10160;
    let t32074 = 0.23712505529730124666e-2 * t32073;
    let t32076 = t6525 * t8026 * t2317;
    let t32077 = 0.23712505529730124666e-2 * t32076;
    let t32079 = t23983 * t1365 * t25575;
    let t32080 = 0.23712505529730124666e-2 * t32079;
    let t32081 = t4382 * t986;
    let t32083 = t9074 * t32081 * t6470;
    let t32084 = 0.82993769354055436331e-2 * t32083;
    let t32085 = t30189 - t32053 + t32055 - t32057 + t32059 + t32062 - t30199 - t32066 - 0.56910013271352299198e-1 * t1063 * t535 * t32067 - t32072 - t30207 - t32074 - t32077 - t32080 - t32084;
    (t32081, t32085)
}
