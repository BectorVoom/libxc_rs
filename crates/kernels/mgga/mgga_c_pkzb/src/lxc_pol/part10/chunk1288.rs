//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1288/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1288<F: Float>(t2922: F, t774: F, t9567: F, t2027: F, t3542: F, t1137: F, t2031: F, t2039: F, t2104: F, t2105: F, t21607: F, t21714: F, t21718: F, t21746: F, t21749: F, t21752: F, t21755: F, t21758: F, t25221: F, t287: F, t2899: F, t5693: F, t7350: F, t759: F, t761: F, t7702: F, t7707: F, t9161: F, t9264: F, t9325: F) -> (F,) {
    let t25485 = t2922 * t774 * t9567;
    let t25492 = t3542 * t2027;
    let t25513 = 0.85748036236139473944e-3 * t2922 * t25221 * t7702 + 0.3811023832717309953e-3 * t21714 + 0.6097638132347695925e-2 * t21718 + 0.13719685797782315831e-1 * t21607 * t9325 - 0.28582678745379824648e-3 * t25485 - 0.85748036236139473944e-3 * t2104 * t2105 * t1137 * t287 * t7350 + 0.25724410870841842184e-2 * t2899 * t5693 * t25492 * t2031 - 0.12862205435420921092e-2 * t2922 * t5693 * t25492 * t2039 - 0.11433071498151929859e-2 * t21746 - 0.57165357490759649296e-3 * t21749 - 0.57165357490759649296e-3 * t21752 - 0.28582678745379824648e-3 * t21755 + 0.28582678745379824648e-3 * t21758 - 0.85748036236139473944e-3 * t2104 * t2105 * t9161 * t759 * t761 + 0.18292914397043087775e-1 * t7707 * t9264;
    (t25513,)
}
