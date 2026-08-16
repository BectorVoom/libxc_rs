//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2668/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2668<F: Float>(t182: F, t54374: F, t39510: F, t39512: F, t39514: F, t39522: F, t39530: F, t39532: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F) -> (F, F, F, F, F, F, F, F) {
    let t54419 = F::cast_from(0.19751673498613801407e-1_f64) * t54374 * t182;
    let t54420 = F::cast_from(0.32530743900905219526e-1_f64) * t39510;
    let t54421 = F::cast_from(0.14447919941302971323e1_f64) * t39512;
    let t54422 = F::cast_from(0.65061487801810439052e-1_f64) * t39514;
    let t54423 = F::cast_from(0.97592231702715658578e-1_f64) * t39522;
    let t54424 = F::cast_from(0.51947577317044391277e2_f64) * t39530;
    let t54425 = F::cast_from(0.10526802520742363173e2_f64) * t39532;
    let t54426 = -t39496 + t54419 + t39499 + t39502 - t39505 - t39508 + t54420 + t54421 - t54422 + t39518 - t39521 - t54423 - t39529 - t54424 - t54425 + t39539;
    (t54419, t54420, t54421, t54422, t54423, t54424, t54425, t54426)
}
