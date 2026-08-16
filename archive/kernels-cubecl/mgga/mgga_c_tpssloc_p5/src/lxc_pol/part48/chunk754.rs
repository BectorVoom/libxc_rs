//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 754/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk754<F: Float>(t1268: F, t22479: F, t12461: F, t3698: F, t2019: F, t1983: F, t12521: F, t1873: F, t12524: F, t7015: F, t3938: F, t6534: F) -> (F, F, F, F, F, F, F) {
    let t23854 = F::cast_from(2.0_f64) * t1268 * t22479;
    let t23857 = t12461 * t3698;
    let t23858 = t2019 * t23857;
    let t23860 = F::cast_from(2.0_f64) * t1983 * t23858;
    let t23886 = F::cast_from(0.135e2_f64) * t12521 * t1873;
    let t23888 = F::cast_from(54.0_f64) * t12524 * t7015;
    let t23890 = F::cast_from(27.0_f64) * t3938 * t6534;
    (t23854, t23857, t23858, t23860, t23886, t23888, t23890)
}
