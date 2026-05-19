//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1348/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1348<F: Float>(t10140: F, t4614: F, t597: F, t10359: F, t4953: F, t10364: F, t1562: F, t1445: F, t31711: F, t31866: F, t26343: F, t895: F) -> (F, F, F, F, F, F) {
    let t34994 = F::cast_from(0.30674340763136599742e2_f64) * t597 * t4614 * t10140;
    let t34996 = F::cast_from(0.18404604457881959845e2_f64) * t4953 * t10359;
    let t34999 = F::cast_from(0.18404604457881959845e2_f64) * t1562 * t4614 * t10364;
    let t35021 = F::cast_from(0.23005755572352449806e2_f64) * t597 * t1445 * t31711;
    let t35024 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t31866;
    let t35027 = F::cast_from(0.35750489951850426669e0_f64) * t895 * t26343;
    (t34994, t34996, t34999, t35021, t35024, t35027)
}
