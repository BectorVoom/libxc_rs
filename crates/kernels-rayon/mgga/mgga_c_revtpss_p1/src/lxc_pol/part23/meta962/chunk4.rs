//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3254/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3254(t22881: f64, t9962: f64, t13783: f64, t13790: f64, t1398: f64, t1883: f64, t22274: f64, t3934: f64, t46596: f64, t46620: f64, t46645: f64, t46652: f64, t48487: f64, t48798: f64, t5671: f64, t5673: f64, t5675: f64, t6836: f64, t73859: f64, t73923: f64, t73927: f64, t73929: f64, t73951: f64, t73953: f64, t73963: f64, t73975: f64, t73985: f64, t85609: f64) -> f64 {
    let t85705 = t9962 * t22881;
    let t85709 = 0.77173232612525526552e-1_f64 * t3934 * t48798 * t1883 * t22274 + 0.15246000842785598467e-3_f64 * t73859 + 0.42874018118069736972e-3_f64 * t5671 * t5673 * t85609 * t5675 + 0.25724410870841842184e-1_f64 * t5671 * t13783 * t13790 * t6836 * t1398 + 0.15117061203111996148e0_f64 * t46596 + 0.72250660161932334527e-3_f64 * t46620 - 0.51384669507166276316e-2_f64 * t46645 - 0.85748036236139473944e-4_f64 * t73923 + 0.21437009059034868486e-4_f64 * t73927 + 0.34013387707001991332e-1_f64 * t73929 - 0.42874018118069736972e-4_f64 * t73951 + 0.4065600224742826258e-3_f64 * t73953 - 0.38115002106963996168e-4_f64 * t73963 + 0.85748036236139473945e-3_f64 * t73975 + 0.3001181268264881588e-2_f64 * t85705 + 455.0_f64 / 648.0_f64 * t46652 - 0.12862205435420921092e-3_f64 * t73985 + t48487;
    t85709
}
