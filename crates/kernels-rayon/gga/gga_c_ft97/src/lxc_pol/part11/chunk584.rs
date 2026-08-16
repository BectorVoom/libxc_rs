//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 584/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk584(t425: f64, t8130: f64, t1725: f64, t1732: f64, t1748: f64, t1739: f64, t8074: f64, t8079: f64, t8082: f64, t8086: f64, t8091: f64, t8094: f64, t8096: f64, t8099: f64, t8104: f64, t8107: f64, t8110: f64, t8113: f64, t8116: f64, t8123: f64, t8127: f64) -> (f64, f64, f64, f64, f64) {
    let t8131 = t8130 * t425;
    let t8133 = t1725 * t1732;
    let t8135 = t1725 * t1748;
    let t8137 = t1725 * t1739;
    let t8139 = 0.3404992446913580247e-1_f64 * t8074 + t8079 - 0.87394806137448559671e0_f64 * t8082 + 0.18727458458024691358e0_f64 * t8086 + 0.38306165027777777778e-1_f64 * t8091 - 0.38306165027777777778e-1_f64 * t8094 + 0.10214977340740740741e0_f64 * t8096 - 0.12768721675925925926e-1_f64 * t8099 - 0.51074886703703703704e-1_f64 * t8104 + 0.25537443351851851852e-1_f64 * t8107 - 0.42562405586419753086e-2_f64 * t8110 + 0.63843608379629629629e-2_f64 * t8113 + 0.85124811172839506172e-2_f64 * t8116 + 0.19862455940329218107e-1_f64 * t8123 + 0.6384360837962962963e-2_f64 * t8127 + 0.18727458458024691358e0_f64 * t8131 - 0.3404992446913580247e-1_f64 * t8133 - 0.51074886703703703705e-1_f64 * t8135 - 0.68099848938271604939e-1_f64 * t8137;
    (t8131, t8133, t8135, t8137, t8139)
}
