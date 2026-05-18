//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1189/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1189<F: Float>(t10227: F, t23927: F, t10276: F, t4141: F, t2321: F, t27071: F, t9074: F, t10555: F, t169: F, t31548: F, t6490: F, t1365: F, t25735: F, t6525: F) -> (F, F, F, F, F) {
    let t32009 = t23927 * t10227;
    let t32010 = F::new(0.23712505529730124666e-2) * t32009;
    let t32012 = F::new(0.9485002211892049866e-2) * t4141 * t10276;
    let t32020 = t9074 * t27071 * t2321;
    let t32021 = F::new(0.11856252764865062333e-2) * t32020;
    let t32025 = F::new(0.68292015925622759036e0) * t31548 * t10555 * t169 * t6490;
    let t32027 = t6525 * t1365 * t25735;
    (t32010, t32012, t32021, t32025, t32027)
}
