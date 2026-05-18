//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 928/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk928<F: Float>(t68: F, t9819: F, t1287: F, t1292: F, t2488: F, t1290: F, t1342: F, t1451: F, t2513: F, t2531: F, t311: F, t410: F, t4945: F, t4950: F, t4981: F, t4983: F, t4997: F, t5000: F, t5014: F, t5632: F, t6121: F, t6138: F, t9764: F, t9770: F, t9777: F, t9798: F, t9816: F) -> (F, F, F) {
    let t9820 = t9819 * t68;
    let t9821 = t9820 * t1287;
    let t9823 = t2488 * t1292;
    let t9825 = F::new(1.8805371096875316) * t9764 * t311 - F::new(19.489173774580152) * t6138 * t2513 - F::new(19.489173774580152) * t1290 * t9770 - F::new(1.8805371096875316) * t6121 * t2513 - F::new(1.8805371096875316) * t1342 * t9770 + F::new(7.35994946043302) * t9777 - t4945 - F::new(1.6457779058161184) * t4950 + t4981 - F::new(3.600163427964126) * t4983 - t4997 - t5000 - t5014 - F::new(2.9824072957409817) * t2531 * t5632 - F::new(2.9824072957409817) * t9798 * t1451 - F::new(1.8805371096875316) * t9816 * t410 + F::new(22.07984838129906) * t9821 + F::new(22.07984838129906) * t9823;
    (t9821, t9823, t9825)
}
