//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 997/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk997<F: Float>(t1690: F, t5014: F, t6790: F, t444: F, t4995: F, t1419: F, t1416: F, t4986: F, t1408: F, t1417: F, t1420: F, t21306: F, t21393: F, t238: F, t24342: F, t24361: F, t24372: F, t27527: F, t27642: F, t27677: F, t27704: F, t27717: F, t30603: F, t30653: F, t30656: F, t30660: F, t30667: F, t30671: F, t30677: F, t30680: F, t30685: F, t30690: F, t30696: F, t3759: F, t4987: F, t6023: F, t6034: F, t6035: F, t6759: F, t6804: F) -> (F, F, F, F, F) {
    let t30700 = t1690 * t6790 * t5014;
    let t30708 = t444 * t4995;
    let t30709 = t30708 * t1419;
    let t30712 = t4986 * t1416;
    let t30715 = 0.10338048737805743098e-3 * t27527 * t6023 * t30603 - 0.44455354858818847408e-2 * t4987 * t1408 + 0.46509801892875584e-2 * t3759 * t30653 - 0.14836531933660919214e-4 * t24372 * t6035 * t30656 + 0.25537443351851851852e-1 * t24361 * t6035 * t30660 - 0.11877414311451622578e-2 * t6034 * t27642 * t6804 - 0.44540303667943584666e-4 * t6034 * t6035 * t30667 - 0.10417318313778431088e-5 * t30671 * t30677 - 0.46509801892875584e-1 * t3759 * t30680 - 0.23254900946437792e-1 * t3759 * t30685 + 0.38731446812548799881e-3 * t3759 * t30690 - 0.46509801892875584e-1 * t27704 * t6759 - 0.75080154872671831175e-1 * t238 * t30696 + 0.89591295428265718861e-3 * t238 * t30700 + 0.14846767889314528222e-3 * t27677 - 0.60102574844279699039e-6 * t21393 * t24342 - 0.2370952259137005195e-1 * t27717 * t21306 - 0.18727458458024691358e0 * t1417 * t30709 - 0.38306165027777777778e-1 * t30712 * t1420;
    (t30700, t30708, t30709, t30712, t30715)
}
