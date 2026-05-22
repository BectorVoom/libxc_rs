;; Disable flycheck in this workspace — cargo check --workspace --all-targets
;; OOMs the machine (281 kernel crates, 3-7 GB each under CubeCL proc-macro).
((nil . ((eval . (when (fboundp 'flycheck-mode) (flycheck-mode -1))))))
