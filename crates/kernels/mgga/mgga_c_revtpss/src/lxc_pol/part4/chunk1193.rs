//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1193/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1193<F: Float>(t10769: F, t828: F, t1544: F, t836: F, t2749: F, t2746: F, t14494: F, t775: F, t14586: F, t10693: F, t10706: F, t10711: F, t10713: F, t10717: F, t10719: F, t10723: F, t10730: F, t10734: F, t10742: F, t2745: F, t4362: F) -> F {
    let t14785 = t10769 * t828;
    let t14786 = t1544 * t836;
    let t14787 = t14786 * t2749;
    let t14788 = t14785 * t14787;
    let t14791 = t2746 * t828;
    let t14792 = t14494 * t2749;
    let t14793 = t14791 * t14792;
    let t14802 = t775 * t836;
    let t14803 = t14586 * t14802;
    let t14804 = t14791 * t14803;
    let t14811 = -F::new(0.85748036236139473944e-2) * t2745 * t14788 + F::new(0.17149607247227894789e-2) * t2745 * t14793 - F::new(0.20007875121765877254e-1) * t10693 + F::new(0.25410001404642664112e-3) * t10706 + F::new(0.71456696863449561619e-5) * t10711 + F::new(0.40015750243531754508e-2) * t10713 + F::new(0.10841600599314203354e-2) * t10717 - F::new(0.15244095330869239812e-3) * t10719 - F::new(0.34299214494455789578e-2) * t4362 * t14804 - F::new(0.45351183609335988442e-1) * t10723 - F::new(0.14291339372689912324e-4) * t10730 + F::new(0.71456696863449561619e-5) * t10734 - F::new(0.50820002809285328224e-4) * t10742;
    t14811
}
