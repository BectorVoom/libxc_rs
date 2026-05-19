//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1370/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1370<F: Float>(t5981: F, t8931: F, t1507: F, t16005: F, t16009: F, t16030: F, t16403: F, t16417: F, t16548: F, t17649: F, t17656: F, t17669: F, t17673: F, t17676: F, t17685: F, t1991: F, t2018: F, t2429: F, t3782: F, t3816: F, t4193: F, t4202: F, t5133: F, t5459: F, t5482: F, t5947: F, t5958: F) -> F {
    let t17686 = t8931 * t5981;
    let t17688 = F::cast_from(0.26531111111111111111e0_f64) * t5133 * t17649 + F::new(0.9286875e-2) * t4193 * t1991 + F::new(0.1857375e-1) * t1507 * t5482 + F::new(0.9286875e-2) * t17656 * t5459 - F::new(0.371475e-1) * t5958 * t16403 + F::new(0.24765e-1) * t5958 * t16548 + F::new(0.9286875e-2) * t5947 * t16005 + F::new(0.46434375e-2) * t5947 * t16009 - F::new(0.1857375e-1) * t4202 * t16030 - F::new(0.9286875e-2) * t5947 * t17669 + F::cast_from(0.10612444444444444444e0_f64) * t2429 * t17673 + F::cast_from(0.5895802469135802469e-2_f64) * t17676 + F::new(0.123825e-1) * t2018 * t3816 + F::new(0.46434375e-2) * t2018 * t3782 + F::new(0.1857375e-1) * t4202 * t16417 - t17685 - F::cast_from(0.88437037037037037036e-1_f64) * t17686;
    t17688
}
