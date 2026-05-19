//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 879/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk879<F: Float>(t28369: F, t4726: F, t1856: F, t28389: F, t706: F, t1835: F, t1919: F, t28373: F, t11633: F, t11635: F, t158: F, t165: F, t173: F, t23255: F, t23259: F, t23261: F, t23263: F) -> F {
    let t28676 = t4726 * t28369;
    let t28679 = t1856 * t28389;
    let t28682 = t706 * t28369;
    let t28685 = t1835 * t28389;
    let t28688 = t1919 * t28369;
    let t28691 = t706 * t28373;
    let t28694 = F::cast_from(0.79249999999999999999e-2_f64) * t23255 + F::cast_from(0.71734315950379065738e-1_f64) * t23259 - F::cast_from(0.35867157975189532869e-1_f64) * t23261 + F::cast_from(0.31077233446777841256e-3_f64) * t23263 - F::cast_from(0.17611111111111111111e-3_f64) * t165 * t28676 + F::new(0.50413125e-5) * t173 * t28679 + F::cast_from(0.22405833333333333333e-5_f64) * t173 * t28682 - F::new(0.3513e-2) * t158 * t28685 + F::cast_from(0.78066666666666666667e-3_f64) * t158 * t28688 - F::new(0.7026e-2) * t158 * t28691 + t11633 - t11635;
    t28694
}
