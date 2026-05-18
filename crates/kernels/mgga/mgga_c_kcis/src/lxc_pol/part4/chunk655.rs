//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 655/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk655<F: Float>(t174: F, t236: F, t3703: F, t233: F, t2645: F, t447: F, t637: F, t446: F, t1300: F, t1640: F, t1385: F, t503: F, t3187: F, t3188: F, t8: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t3704 = t236 * t3703;
    let t3705 = t233 * t3704;
    let t3706 = t3705 / F::new(16.0);
    let t3707 = piecewise3::<f64>(t175, F::new(0.0), t2645);
    let t3708 = t447 * t3707;
    let t3709 = t3708 * t637;
    let t3710 = t446 * t3709;
    let t3711 = t3710 / F::new(16.0);
    let t3712 = t1300 * t1640;
    let t3713 = t446 * t3712;
    let t3714 = t3713 / F::new(8.0);
    let t3715 = t1385 * t1385;
    let t3716 = t503 * t503;
    let t3717 = F::new(1.0) / t3716;
    let t3718 = t3715 * t3717;
    let t3722 = t2645 * t8 - t3187 + t3188;
    (t3706, t3708, t3709, t3711, t3712, t3714, t3715, t3716, t3717, t3718, t3722)
}
