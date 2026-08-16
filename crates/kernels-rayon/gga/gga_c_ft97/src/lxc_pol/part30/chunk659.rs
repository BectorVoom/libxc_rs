//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 659/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk659(t14763: f64, t6241: f64, t1472: f64, t25070: f64, t25077: f64, t25106: f64, t27502: f64, t27634: f64, t27662: f64, t28540: f64, t28544: f64, t28548: f64, t28552: f64, t28558: f64, t28562: f64, t28567: f64, t28572: f64, t28575: f64, t28577: f64, t4104: f64, t6035: f64, t7000: f64) -> f64 {
    let t28579 = t14763 * t6241;
    let t28582 = 0.22653425206514361674e0_f64 * t1472 * t28540 + 0.24163653553615319118e1_f64 * t4104 * t28544 + 0.33339000546296296298e-1_f64 * t25077 * t6035 * t28548 + 0.33339000546296296298e-1_f64 * t28552 * t27502 - 0.22226000364197530865e-1_f64 * t28552 * t27634 - 0.40279602951224778277e-1_f64 * t28558 * t27662 + 0.33339000546296296297e-1_f64 * t25077 * t6035 * t28562 - 0.33339000546296296298e-1_f64 * t25070 * t6035 * t28567 - 0.55565000910493827163e-2_f64 * t25106 - 0.40279602951224778277e-1_f64 * t28572 - 0.33339000546296296297e-1_f64 * t28575 + 0.40279602951224778277e-1_f64 * t28577 - 0.10001700163888888889e0_f64 * t28579 * t7000;
    t28582
}
