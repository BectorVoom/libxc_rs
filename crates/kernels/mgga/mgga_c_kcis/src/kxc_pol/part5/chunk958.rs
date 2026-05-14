//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 958/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk958<F: Float>(t3728: F, t5749: F, t5753: F, t11913: F, t5645: F, t1984: F, t3245: F, t20: F, t492: F, t2194: F, t1369: F, t3999: F, t110: F, t1939: F, t493: F, t1930: F, t3974: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16804 = t3728 * t5749;
    let t16805 = 0.33163888888888888888e-2 * t16804;
    let t16806 = t3728 * t5753;
    let t16808 = t11913 * t5645;
    let t16809 = 0.22109259259259259258e-2 * t16808;
    let t16820 = t3245 * t1984;
    let t16829 = t492 * t20;
    let t16830 = t16829 * t2194;
    let t16831 = t1369 * t3999;
    let t16841 = t110 * t1939;
    let t16842 = t493 * t16841;
    let t16845 = t1930 * t3974 / 54.0;
    (t16804, t16805, t16806, t16808, t16809, t16820, t16830, t16831, t16842, t16845)
}
