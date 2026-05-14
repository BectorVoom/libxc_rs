//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1184/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1184<F: Float>(t108639: F, t7009: F, t4125: F, t703: F, t2691: F, t98519: F, t1407: F, t70458: F, t6242: F, t6999: F, t96535: F, t108635: F, t108969: F, t14770: F, t14810: F, t231: F, t25077: F, t28558: F, t35870: F, t6035: F, t6045: F, t6249: F, t6256: F, t684: F, t70786: F, t98432: F, t98438: F, t98446: F) -> (F, F) {
    let t111908 = t7009 * t108639;
    let t111910 = t703 * t4125;
    let t111915 = t2691 * t98519;
    let t111922 = t1407 * t70458;
    let t111935 = t6242 * t96535 * t6999;
    let t111937 = 0.26853068634149852185e-1 * t111908 + 0.66678001092592592594e-1 * t25077 * t6035 * t111910 * t684 - 0.12002040196666666667e1 * t111915 * t6035 * t35870 * t14770 + 0.33339000546296296298e-1 * t6256 * t108635 + 0.18122740165211489339e1 * t70786 * t111922 + 0.44452000728395061731e-1 * t98432 - 0.33339000546296296298e-1 * t98438 - 0.22226000364197530866e-1 * t98446 + 0.80559205902449556554e-1 * t28558 * t108969 + 0.10001700163888888889e0 * t6249 * t6045 * t231 * t14810 + 0.22226000364197530865e-1 * t111935;
    (t111922, t111937)
}
