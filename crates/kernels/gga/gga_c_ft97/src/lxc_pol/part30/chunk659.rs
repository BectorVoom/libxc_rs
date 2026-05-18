//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 659/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk659<F: Float>(t14763: F, t6241: F, t1472: F, t25070: F, t25077: F, t25106: F, t27502: F, t27634: F, t27662: F, t28540: F, t28544: F, t28548: F, t28552: F, t28558: F, t28562: F, t28567: F, t28572: F, t28575: F, t28577: F, t4104: F, t6035: F, t7000: F) -> F {
    let t28579 = t14763 * t6241;
    let t28582 = F::new(0.22653425206514361674e0) * t1472 * t28540 + F::new(0.24163653553615319118e1) * t4104 * t28544 + F::new(0.33339000546296296298e-1) * t25077 * t6035 * t28548 + F::new(0.33339000546296296298e-1) * t28552 * t27502 - F::new(0.22226000364197530865e-1) * t28552 * t27634 - F::new(0.40279602951224778277e-1) * t28558 * t27662 + F::new(0.33339000546296296297e-1) * t25077 * t6035 * t28562 - F::new(0.33339000546296296298e-1) * t25070 * t6035 * t28567 - F::new(0.55565000910493827163e-2) * t25106 - F::new(0.40279602951224778277e-1) * t28572 - F::new(0.33339000546296296297e-1) * t28575 + F::new(0.40279602951224778277e-1) * t28577 - F::new(0.10001700163888888889e0) * t28579 * t7000;
    t28582
}
