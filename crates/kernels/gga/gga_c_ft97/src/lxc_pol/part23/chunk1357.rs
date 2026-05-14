//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1357/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1357<F: Float>(t126018: F, t1486: F, t193: F, t2781: F, t24330: F, t31385: F, t6242: F, t123367: F, t6256: F, t24378: F, t25077: F, t31446: F, t5260: F, t703: F, t108817: F, t108972: F, t111844: F, t111861: F, t111868: F, t1196: F, t1208: F, t123261: F, t2347: F, t2360: F, t25070: F, t28547: F, t28558: F, t28598: F, t28603: F, t3746: F, t3886: F, t6035: F, t684: F, t98544: F, t98545: F) -> (F, F) {
    let t127042 = t1486 * t193 * t2781 * t126018;
    let t127077 = t6242 * t24330 * t31385;
    let t127079 = t6256 * t123367;
    let t127082 = t25077 * t24378 * t31446;
    let t127084 = t703 * t5260;
    let t127089 = -0.13335600218518518519e0 * t25077 * t98545 * t1208 * t2360 * t3886 + 0.8890400145679012346e-1 * t25077 * t108972 * t1208 * t2347 * t3886 + 0.13335600218518518519e0 * t98544 * t98545 * t111844 * t28547 + 0.13335600218518518519e0 * t25070 * t108817 * t28598 * t3746 - 0.1611184118048991131e0 * t28603 * t123261 + 0.1611184118048991131e0 * t28558 * t123261 + t111861 + t111868 + 0.13335600218518518519e0 * t25070 * t98545 * t1196 * t2360 * t3886 - 0.8890400145679012346e-1 * t25070 * t108972 * t1196 * t2347 * t3886 - 0.33339000546296296297e-1 * t127077 + 0.29634667152263374487e-1 * t127079 + 0.22226000364197530865e-1 * t127082 - 0.33339000546296296297e-1 * t25070 * t6035 * t127084 * t684;
    (t127042, t127089)
}
