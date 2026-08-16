//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3254/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3254<F: Float>(t22881: F, t9962: F, t13783: F, t13790: F, t1398: F, t1883: F, t22274: F, t3934: F, t46596: F, t46620: F, t46645: F, t46652: F, t48487: F, t48798: F, t5671: F, t5673: F, t5675: F, t6836: F, t73859: F, t73923: F, t73927: F, t73929: F, t73951: F, t73953: F, t73963: F, t73975: F, t73985: F, t85609: F) -> F {
    let t85705 = t9962 * t22881;
    let t85709 = F::cast_from(0.77173232612525526552e-1_f64) * t3934 * t48798 * t1883 * t22274 + F::cast_from(0.15246000842785598467e-3_f64) * t73859 + F::cast_from(0.42874018118069736972e-3_f64) * t5671 * t5673 * t85609 * t5675 + F::cast_from(0.25724410870841842184e-1_f64) * t5671 * t13783 * t13790 * t6836 * t1398 + F::cast_from(0.15117061203111996148e0_f64) * t46596 + F::cast_from(0.72250660161932334527e-3_f64) * t46620 - F::cast_from(0.51384669507166276316e-2_f64) * t46645 - F::cast_from(0.85748036236139473944e-4_f64) * t73923 + F::cast_from(0.21437009059034868486e-4_f64) * t73927 + F::cast_from(0.34013387707001991332e-1_f64) * t73929 - F::cast_from(0.42874018118069736972e-4_f64) * t73951 + F::cast_from(0.4065600224742826258e-3_f64) * t73953 - F::cast_from(0.38115002106963996168e-4_f64) * t73963 + F::cast_from(0.85748036236139473945e-3_f64) * t73975 + F::cast_from(0.3001181268264881588e-2_f64) * t85705 + F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t46652 - F::cast_from(0.12862205435420921092e-3_f64) * t73985 + t48487;
    t85709
}
