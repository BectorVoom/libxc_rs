//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 918/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk918<F: Float>(t1208: F, t703: F, t684: F, t811: F, t992: F, t704: F, t27575: F, t7009: F, t24330: F, t6999: F, t6242: F, t7006: F, t14763: F, t6241: F, t1472: F, t25070: F, t25077: F, t25106: F, t27502: F, t27634: F, t27662: F, t28540: F, t28544: F, t28548: F, t28552: F, t28558: F, t4104: F, t6035: F, t7000: F) -> (F, F, F, F, F, F) {
    let t28561 = t703 * t1208;
    let t28562 = t28561 * t684;
    let t28566 = t992 * t811;
    let t28567 = t704 * t28566;
    let t28572 = t7009 * t27575;
    let t28574 = t24330 * t6999;
    let t28575 = t6242 * t28574;
    let t28577 = t7006 * t27575;
    let t28579 = t14763 * t6241;
    let t28582 = 0.22653425206514361674e0 * t1472 * t28540 + 0.24163653553615319118e1 * t4104 * t28544 + 0.33339000546296296298e-1 * t25077 * t6035 * t28548 + 0.33339000546296296298e-1 * t28552 * t27502 - 0.22226000364197530865e-1 * t28552 * t27634 - 0.40279602951224778277e-1 * t28558 * t27662 + 0.33339000546296296297e-1 * t25077 * t6035 * t28562 - 0.33339000546296296298e-1 * t25070 * t6035 * t28567 - 0.55565000910493827163e-2 * t25106 - 0.40279602951224778277e-1 * t28572 - 0.33339000546296296297e-1 * t28575 + 0.40279602951224778277e-1 * t28577 - 0.10001700163888888889e0 * t28579 * t7000;
    (t28561, t28562, t28567, t28574, t28579, t28582)
}
