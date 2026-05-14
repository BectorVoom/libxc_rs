//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 800/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk800<F: Float>(t1608: F, t1630: F, t7843: F, t11121: F, t11209: F, t1594: F, t1603: F, t1617: F, t1620: F, t1624: F, t1656: F, t1665: F, t1698: F, t3066: F, t35: F, t372: F, t374: F, t37506: F, t37526: F, t37552: F, t37558: F, t37570: F, t37574: F, t37670: F, t38037: F, t38111: F, t38117: F, t38120: F, t38129: F, t38146: F, t38150: F, t38155: F, t384: F, t39: F, t534: F, t6: F, t7914: F, t7978: F, t7982: F, t7985: F, t7989: F, t7992: F, t7993: F, t8000: F, t8003: F, t8068: F) -> (F,) {
    let t38159 = t1608 * t7843 * t1630;
    let t38175 = 0.16223712540858999423e-3 * t38037 * t7989 - 0.32447425081717998847e-3 * t38037 * t7985 - 0.11627450473218896e-1 * t372 * t374 * t38111 * t35 + 0.81118562704294997116e-3 * t7982 * t38117 - 0.27020878774141382658e-4 * t38120 * t7985 + 0.16864243845320605903e-2 * t1665 * t1698 - 0.20265659080606036994e-4 * t11209 * t7993 - 0.30589033253692324537e-6 * t38129 * t37506 - 0.27568129967481981592e-3 * t1617 * t7978 * t6 * t1620 + 0.19232823950169503692e-4 * t11121 * t7992 * t6 * t1620 - 0.9804408003987596673e-5 * t8000 * t1656 * t39 * t8003 + 0.16329414088222212441e-6 * t38146 * t37670 - 0.16540877980489188955e-2 * t38150 * t37552 * t3066 - 0.66163511921956755822e-4 * t38155 * t37506 - 0.16540877980489188955e-3 * t38159 * t37526 - 0.93019603785751168e-1 * t1603 * t374 * t384 * t8068 + 0.23238868087529279928e-2 * t1603 * t1594 * t37570 + 0.12901581267952785412e-4 * t1624 * t534 * t37574 - 0.2232470490858028032e-1 * t1603 * t7914 * t37558;
    (t38175,)
}
