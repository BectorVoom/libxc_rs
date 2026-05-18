//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 861/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk861<F: Float>(t35: F, t37640: F, t11109: F, t11351: F, t1594: F, t1603: F, t1617: F, t1626: F, t1631: F, t1633: F, t1656: F, t1657: F, t1660: F, t1683: F, t1685: F, t1687: F, t1701: F, t1712: F, t2021: F, t2035: F, t2037: F, t22834: F, t3019: F, t3020: F, t372: F, t374: F, t37611: F, t37614: F, t37622: F, t37628: F, t399: F, t401: F, t5545: F, t7202: F, t77: F, t7833: F, t7877: F, t7879: F, t7919: F, t7977: F, t7982: F, t7993: F, t8018: F, t8070: F, t8174: F) -> (F, F) {
    let t37641 = t37640 * t35;
    let t37666 = -F::new(0.279058811357253504e0) * t22834 * t8174 + F::new(0.279058811357253504e0) * t37611 * t1626 + F::new(0.22023512095983737145e1) * t5545 * t1701 * t37614 * t401 + F::new(0.23238868087529279928e-2) * t7919 * t1660 - F::new(0.40559281352147498558e-3) * t7982 * t37622 - F::new(0.139529405678626752e0) * t7919 * t1657 + F::new(0.69764702839313376e-2) * t372 * t1631 * t37628 - F::new(0.24335568811288499135e-3) * t11109 * t7993 + F::new(0.27039520901431665705e-3) * t3019 * t3020 * t77 * t7977 + F::new(0.279058811357253504e-1) * t7919 * t1633 + F::new(0.38704743803858356237e-5) * t372 * t2021 * t37641 - F::new(0.139529405678626752e0) * t1603 * t374 * t1656 * t1685 + F::new(0.58097170218823199823e-3) * t372 * t1594 * t37628 + F::new(0.474190451827401039e-1) * t8070 * t399 + F::new(0.279058811357253504e0) * t7877 * t11351 * t7879 + F::new(0.16864243845320605903e-2) * t7202 * t2035 * t2037 * t1712 - F::new(0.45048092923603098705e0) * t1687 * t1683 + F::new(0.26189723469107882512e-2) * t1617 * t7833 * t8018;
    (t37641, t37666)
}
