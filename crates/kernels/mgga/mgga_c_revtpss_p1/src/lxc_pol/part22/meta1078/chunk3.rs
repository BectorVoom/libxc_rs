//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3863/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3863<F: Float>(t13847: F, t1399: F, t73856: F, t9816: F, t22298: F, t48100: F, t22129: F, t2713: F, t3964: F, t22046: F, t22079: F, t3829: F, t3934: F, t4057: F, t48548: F, t48553: F, t48557: F, t48563: F, t48565: F, t5671: F, t5673: F, t6883: F, t73847: F, t800: F, t9748: F, t9840: F) -> F {
    let t74249 = t9816 * t13847 * t73856 * t1399;
    let t74257 = t9816 * t48100 * t22298;
    let t74264 = t3964 * t2713 * t22129;
    let t74266 = F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t5673 * t22046 * t9840 - F::cast_from(0.42874018118069736972e-3_f64) * t3934 * t5673 * t73847 * t1399 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t22079 * t4057 - F::cast_from(0.25410001404642664112e-4_f64) * t74249 + F::cast_from(0.2032800112371413129e-3_f64) * t48548 - t9748 * t800 * t6883 * t3829 / F::new(4.0) + F::cast_from(0.2032800112371413129e-3_f64) * t74257 - F::cast_from(0.11433071498151929859e-3_f64) * t48553 - F::cast_from(0.57165357490759649296e-4_f64) * t48557 - F::cast_from(0.40164115440237189888e-6_f64) * t48563 - F::cast_from(0.40015750243531754508e-1_f64) * t48565 + F::cast_from(0.90357964994909313586e-4_f64) * t74264;
    t74266
}
