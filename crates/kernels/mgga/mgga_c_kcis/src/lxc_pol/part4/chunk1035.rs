//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1035/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1035<F: Float>(t169: F, t4535: F, t911: F, t1300: F, t6260: F, t446: F, t13003: F, t1646: F, t167: F, t2629: F, t160: F, t171: F, t2630: F, t2635: F, t4510: F, t4513: F, t740: F, t829: F, zeta_threshold: F) -> (F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t13057 = t911 * t4535;
    let t13059 = t1300 * t6260;
    let t13060 = t446 * t13059;
    let t13062 = t13003 * t1646;
    let t13065 = t2629 * t167;
    let t13076 = piecewise3::<f64>(t170, F::new(0.0), -F::new(8.0) / F::new(27.0) * t13062 * t2630 + F::new(16.0) / F::new(9.0) * t13065 * t740 * t829 + F::new(4.0) / F::new(9.0) * t4510 * t2635 + F::new(8.0) / F::new(3.0) * t171 * t740 - F::new(8.0) * t4513 * t160);
    (t13057, t13060, t13076)
}
