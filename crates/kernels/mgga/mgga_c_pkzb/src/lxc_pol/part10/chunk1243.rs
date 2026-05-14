//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1243/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1243<F: Float>(t17053: F, t3418: F, t1721: F, t600: F, t8888: F, t5257: F, t8897: F, t1769: F, t8823: F, t8827: F, t1634: F, t20203: F, t20205: F, t20221: F, t20242: F, t20261: F, t20263: F, t20265: F, t20267: F, t612: F, t6990: F, t8821: F, t8830: F) -> (F, F, F, F) {
    let t24347 = t17053 * t3418;
    let t24350 = t8888 * t1721 * t600;
    let t24370 = t5257 * t8897;
    let t24381 = t1769 * t8823;
    let t24387 = t1769 * t8827;
    let t24394 = 0.40015750243531754508e-1 * t20203 + 0.90702367218671976886e-1 * t20205 - 0.25724410870841842183e-1 * t612 * t6990 * t8830 * t1634 + 0.24009450146119052704e0 * t24381 + 0.18007087609589289528e0 * t612 * t20267 * t8821 * t1634 - 0.80031500487063509015e-1 * t24387 - 35.0 / 54.0 * t20221 + 0.90702367218671976884e-1 * t20242 - 0.22675591804667994222e-1 * t20261 - 0.40015750243531754508e-2 * t20263 - 0.80031500487063509016e-1 * t20265;
    (t24347, t24350, t24370, t24394)
}
