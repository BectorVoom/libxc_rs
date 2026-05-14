//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1021/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1021<F: Float>(t35844: F, t31350: F, t4971: F, t7447: F, t8823: F, t7440: F, t8826: F, t1488: F, t2030: F, t2031: F, t507: F, t7807: F, t31605: F, t35812: F, t35814: F, t35817: F, t35818: F, t35823: F, t35828: F, t35829: F, t35833: F, t35836: F, t35838: F, t35841: F) -> (F,) {
    let t35845 = 0.21437009059034868486e-3 * t35844;
    let t35846 = t31350 * t4971;
    let t35848 = t7447 * t8823;
    let t35849 = 0.84046875e-1 * t35848;
    let t35850 = t7440 * t8826;
    let t35851 = 0.5603125e-1 * t35850;
    let t35853 = t2030 * t1488 * t2031;
    let t35856 = t2030 * t507 * t7807;
    let t35858 = t35812 + 0.38110238327173099531e-2 * t31605 + 0.80031500487063509014e-2 * t35814 + t35817 + 0.14291339372689912324e-3 * t35818 + 0.10718504529517434243e-3 * t35823 + t35828 - 0.80031500487063509014e-2 * t35829 + 0.18868855373762491241e-2 * t35833 - t35836 + t35838 + 0.53592522647587171215e-3 * t35841 + t35845 - 0.17149607247227894789e-1 * t35846 - t35849 - t35851 + 0.22921875e-1 * t35853 + 0.114609375e-1 * t35856;
    (t35858,)
}
