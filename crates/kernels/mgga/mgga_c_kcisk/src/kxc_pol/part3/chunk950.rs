//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 950/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk950<F: Float>(t12983: F, t1375: F, t12868: F, t457: F, t1383: F, t1186: F, t13456: F, t14047: F, t14056: F, t14059: F, t14062: F, t14063: F, t158: F, t165: F, t173: F, t3819: F, t3891: F) -> F {
    let t14066 = t1375 * t12983;
    let t14069 = t1375 * t12868;
    let t14072 = t457 * t12983;
    let t14075 = t1383 * t12868;
    let t14078 = t1186 * t12983;
    let t14081 = F::new(0.403305e-4) * t173 * t14047 - F::cast_from(0.71734315950379065738e-1_f64) * t3819 * t13456 + F::cast_from(0.46615850170166761884e-3_f64) * t3891 * t13456 + t14056 + t14059 - t14062 - F::new(0.30247875e-4) * t173 * t14063 - F::new(0.2016525e-4) * t173 * t14066 + F::new(0.21078e-1) * t158 * t14069 + F::new(0.3513e-2) * t158 * t14072 - F::new(0.4755e-2) * t165 * t14075 - F::new(0.1585e-2) * t165 * t14078;
    t14081
}
