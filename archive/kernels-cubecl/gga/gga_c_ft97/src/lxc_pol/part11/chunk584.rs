//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 584/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk584<F: Float>(t425: F, t8130: F, t1725: F, t1732: F, t1748: F, t1739: F, t8074: F, t8079: F, t8082: F, t8086: F, t8091: F, t8094: F, t8096: F, t8099: F, t8104: F, t8107: F, t8110: F, t8113: F, t8116: F, t8123: F, t8127: F) -> (F, F, F, F, F) {
    let t8131 = t8130 * t425;
    let t8133 = t1725 * t1732;
    let t8135 = t1725 * t1748;
    let t8137 = t1725 * t1739;
    let t8139 = F::cast_from(0.3404992446913580247e-1_f64) * t8074 + t8079 - F::cast_from(0.87394806137448559671e0_f64) * t8082 + F::cast_from(0.18727458458024691358e0_f64) * t8086 + F::cast_from(0.38306165027777777778e-1_f64) * t8091 - F::cast_from(0.38306165027777777778e-1_f64) * t8094 + F::cast_from(0.10214977340740740741e0_f64) * t8096 - F::cast_from(0.12768721675925925926e-1_f64) * t8099 - F::cast_from(0.51074886703703703704e-1_f64) * t8104 + F::cast_from(0.25537443351851851852e-1_f64) * t8107 - F::cast_from(0.42562405586419753086e-2_f64) * t8110 + F::cast_from(0.63843608379629629629e-2_f64) * t8113 + F::cast_from(0.85124811172839506172e-2_f64) * t8116 + F::cast_from(0.19862455940329218107e-1_f64) * t8123 + F::cast_from(0.6384360837962962963e-2_f64) * t8127 + F::cast_from(0.18727458458024691358e0_f64) * t8131 - F::cast_from(0.3404992446913580247e-1_f64) * t8133 - F::cast_from(0.51074886703703703705e-1_f64) * t8135 - F::cast_from(0.68099848938271604939e-1_f64) * t8137;
    (t8131, t8133, t8135, t8137, t8139)
}
